use anyhow::Result;
use quickmark_core::config::{
    config_in_path_or_default, discover_config_with_workspace_or_default, RuleSeverity,
};
use quickmark_core::linter::{MultiRuleLinter, RuleViolation};
use std::env;
use std::path::PathBuf;
use tokio::io::{stdin, stdout};
use tower_lsp::jsonrpc;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    workspace_roots: std::sync::Mutex<Vec<PathBuf>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            workspace_roots: std::sync::Mutex::new(Vec::new()),
        }
    }

    fn lint_document(&self, uri: &Url, content: &str) -> Result<Vec<Diagnostic>> {
        let file_path = uri
            .to_file_path()
            .map_err(|_| anyhow::anyhow!("Invalid file path"))?;

        // Use hierarchical config discovery with workspace roots or fallback to old behavior
        let config = {
            let workspace_roots = self.workspace_roots.lock().unwrap();
            if workspace_roots.is_empty() {
                // Fallback to old behavior if no workspace roots
                let pwd = env::current_dir()?;
                config_in_path_or_default(&pwd)?
            } else {
                discover_config_with_workspace_or_default(&file_path, workspace_roots.clone())?
            }
        };

        let mut linter = MultiRuleLinter::new_for_document(file_path, config.clone(), content);
        let violations = linter.analyze();

        Ok(violations
            .into_iter()
            .map(|violation| self.violation_to_diagnostic(violation, &config))
            .collect())
    }

    fn violation_to_diagnostic(
        &self,
        violation: RuleViolation,
        config: &quickmark_core::config::QuickmarkConfig,
    ) -> Diagnostic {
        // Get severity from configuration
        let rule_severity = config
            .linters
            .severity
            .get(violation.rule().alias)
            .unwrap_or(&RuleSeverity::Warning);

        let severity = match rule_severity {
            RuleSeverity::Error => DiagnosticSeverity::ERROR,
            RuleSeverity::Warning => DiagnosticSeverity::WARNING,
            RuleSeverity::Off => DiagnosticSeverity::HINT, // Shouldn't happen since off rules are filtered
        };

        let range = violation.location();
        Diagnostic {
            range: Range {
                start: Position {
                    line: range.range.start.line as u32,
                    character: range.range.start.character as u32,
                },
                end: Position {
                    line: range.range.end.line as u32,
                    character: range.range.end.character as u32,
                },
            },
            severity: Some(severity),
            code: Some(NumberOrString::String(violation.rule().alias.to_string())),
            source: Some("quickmark".to_string()),
            message: violation.message().to_string(),
            related_information: None,
            tags: None,
            code_description: None,
            data: None,
        }
    }

    async fn publish_diagnostics(&self, uri: Url, content: &str) {
        match self.lint_document(&uri, content) {
            Ok(diagnostics) => {
                self.client
                    .publish_diagnostics(uri, diagnostics, None)
                    .await;
            }
            Err(err) => {
                eprintln!("Failed to lint document: {err}");
            }
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        eprintln!("LSP server initializing with params: {:?}", params.root_uri);

        // Extract workspace roots from initialization parameters
        let mut workspace_roots = Vec::new();

        // Priority 1: workspace_folders from params
        if let Some(folders) = params.workspace_folders {
            for folder in folders {
                if let Ok(path) = folder.uri.to_file_path() {
                    workspace_roots.push(path);
                }
            }
        }

        // Priority 2: root_uri as fallback
        if workspace_roots.is_empty() {
            if let Some(root_uri) = params.root_uri {
                if let Ok(path) = root_uri.to_file_path() {
                    workspace_roots.push(path);
                }
            }
        }

        // Store workspace roots
        {
            let mut stored_roots = self.workspace_roots.lock().unwrap();
            *stored_roots = workspace_roots.clone();
        }

        eprintln!("Workspace roots configured: {:?}", workspace_roots);

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // Explicitly enable full text document synchronization
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: Some(false),
                        will_save_wait_until: Some(false),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                    },
                )),
                // Explicitly enable diagnostics with push model only
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("quickmark".to_string()),
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                    },
                )),
                position_encoding: Some(PositionEncodingKind::UTF16),
                selection_range_provider: Some(SelectionRangeProviderCapability::Simple(false)),
                // Explicitly disable other capabilities we don't support
                hover_provider: Some(HoverProviderCapability::Simple(false)),
                completion_provider: None,
                signature_help_provider: None,
                definition_provider: Some(OneOf::Left(false)),
                type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(false)),
                implementation_provider: Some(ImplementationProviderCapability::Simple(false)),
                references_provider: Some(OneOf::Left(false)),
                document_highlight_provider: Some(OneOf::Left(false)),
                document_symbol_provider: Some(OneOf::Left(false)),
                workspace_symbol_provider: Some(OneOf::Left(false)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(false)),
                code_lens_provider: None,
                document_formatting_provider: Some(OneOf::Left(false)),
                document_range_formatting_provider: Some(OneOf::Left(false)),
                document_on_type_formatting_provider: None,
                rename_provider: Some(OneOf::Left(false)),
                document_link_provider: None,
                color_provider: Some(ColorProviderCapability::Simple(false)),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(false)),
                declaration_provider: Some(DeclarationCapability::Simple(false)),
                execute_command_provider: None,
                workspace: None,
                call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(false)),
                semantic_tokens_provider: None,
                moniker_provider: Some(OneOf::Left(false)),
                linked_editing_range_provider: Some(LinkedEditingRangeServerCapabilities::Simple(
                    false,
                )),
                inline_value_provider: Some(OneOf::Left(false)),
                inlay_hint_provider: Some(OneOf::Left(false)),
                experimental: None,
            },
            server_info: Some(ServerInfo {
                name: "quickmark-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        // Server initialized - ready to accept requests
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        eprintln!("[QUICKMARK] did_open: {} (len: {})", params.text_document.uri, params.text_document.text.len());
        self.publish_diagnostics(params.text_document.uri, &params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        eprintln!("[QUICKMARK] did_change: {} ({} changes)", params.text_document.uri, params.content_changes.len());
        //do nothing on changes, only lint on save
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        eprintln!("[QUICKMARK] did_save: {}", params.text_document.uri);
        eprintln!("[QUICKMARK] text provided: {}", params.text.is_some());
        
        let content = if let Some(text) = params.text {
            eprintln!("[QUICKMARK] using provided text (len: {})", text.len());
            // Use text content provided by the LSP client
            text
        } else {
            eprintln!("[QUICKMARK] no text provided, reading from disk...");
            // Fallback: read file from disk if client doesn't send text content
            // Some LSP clients don't send text despite server requesting include_text: true
            match params.text_document.uri.to_file_path() {
                Ok(path) => {
                    eprintln!("[QUICKMARK] reading file: {:?}", path);
                    match std::fs::read_to_string(&path) {
                        Ok(content) => {
                            eprintln!("[QUICKMARK] successfully read {} chars from disk", content.len());
                            content
                        }
                        Err(err) => {
                            eprintln!("[QUICKMARK] failed to read file {:?}: {}", path, err);
                            return;
                        }
                    }
                },
                Err(_) => {
                    eprintln!("[QUICKMARK] invalid file path: {}", params.text_document.uri);
                    return;
                }
            }
        };

        eprintln!("[QUICKMARK] publishing diagnostics for {} chars", content.len());
        self.publish_diagnostics(params.text_document.uri, &content).await;
        eprintln!("[QUICKMARK] diagnostics published");
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Clear diagnostics
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn diagnostic(
        &self,
        _params: DocumentDiagnosticParams,
    ) -> jsonrpc::Result<DocumentDiagnosticReportResult> {
        // Per Phase 1 requirements: Don't send anything on textDocument/diagnostic event
        // as it doesn't contain the full document's content (caching required for future phases)
        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: vec![], // Empty - we only provide diagnostics via push model
                },
            }),
        ))
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("QuickMark LSP Server starting...");

    let (service, socket) = LspService::new(Backend::new);

    Server::new(stdin(), stdout(), socket).serve(service).await;

    Ok(())
}

#[cfg(test)]
mod tests {

    use quickmark_core::config::{QuickmarkConfig, RuleSeverity};
    use std::collections::HashMap;
    use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range};

    // We'll test the core functionality without needing a full Backend instance

    fn create_test_config_with_severity(rule: &str, severity: RuleSeverity) -> QuickmarkConfig {
        let mut severity_map = HashMap::new();
        severity_map.insert(rule.to_string(), severity);

        QuickmarkConfig {
            linters: quickmark_core::config::LintersTable {
                severity: severity_map,
                ..Default::default()
            },
        }
    }

    // Test violation_to_diagnostic without needing a real Backend
    fn test_violation_to_diagnostic_with_config(
        config: &QuickmarkConfig,
        violation: quickmark_core::linter::RuleViolation,
    ) -> Diagnostic {
        // Get severity from configuration
        let rule_severity = config
            .linters
            .severity
            .get(violation.rule().alias)
            .unwrap_or(&RuleSeverity::Warning);

        let severity = match rule_severity {
            RuleSeverity::Error => DiagnosticSeverity::ERROR,
            RuleSeverity::Warning => DiagnosticSeverity::WARNING,
            RuleSeverity::Off => DiagnosticSeverity::HINT,
        };

        let range = violation.location();
        Diagnostic {
            range: Range {
                start: Position {
                    line: range.range.start.line as u32,
                    character: range.range.start.character as u32,
                },
                end: Position {
                    line: range.range.end.line as u32,
                    character: range.range.end.character as u32,
                },
            },
            severity: Some(severity),
            code: Some(NumberOrString::String(violation.rule().alias.to_string())),
            source: Some("quickmark".to_string()),
            message: violation.message().to_string(),
            related_information: None,
            tags: None,
            code_description: None,
            data: None,
        }
    }

    #[test]
    fn test_violation_to_diagnostic_error_severity() {
        let config = create_test_config_with_severity("line-length", RuleSeverity::Error);

        let violation = quickmark_core::linter::RuleViolation::new(
            &quickmark_core::rules::md013::MD013,
            "Test violation".to_string(),
            std::path::PathBuf::from("/test/file.md"),
            quickmark_core::linter::Range {
                start: quickmark_core::linter::CharPosition {
                    line: 0,
                    character: 0,
                },
                end: quickmark_core::linter::CharPosition {
                    line: 0,
                    character: 10,
                },
            },
        );

        let diagnostic = test_violation_to_diagnostic_with_config(&config, violation);

        assert_eq!(diagnostic.severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(
            diagnostic.code,
            Some(NumberOrString::String("line-length".to_string()))
        );
        assert_eq!(diagnostic.source, Some("quickmark".to_string()));
        assert_eq!(diagnostic.message, "Test violation");
    }

    #[test]
    fn test_violation_to_diagnostic_warning_severity() {
        let config = create_test_config_with_severity("line-length", RuleSeverity::Warning);

        let violation = quickmark_core::linter::RuleViolation::new(
            &quickmark_core::rules::md013::MD013,
            "Test warning".to_string(),
            std::path::PathBuf::from("/test/file.md"),
            quickmark_core::linter::Range {
                start: quickmark_core::linter::CharPosition {
                    line: 2,
                    character: 5,
                },
                end: quickmark_core::linter::CharPosition {
                    line: 2,
                    character: 15,
                },
            },
        );

        let diagnostic = test_violation_to_diagnostic_with_config(&config, violation);

        assert_eq!(diagnostic.severity, Some(DiagnosticSeverity::WARNING));
        assert_eq!(diagnostic.range.start.line, 2);
        assert_eq!(diagnostic.range.start.character, 5);
        assert_eq!(diagnostic.range.end.line, 2);
        assert_eq!(diagnostic.range.end.character, 15);
    }

    #[test]
    fn test_violation_to_diagnostic_default_severity() {
        let config = QuickmarkConfig::default();

        let violation = quickmark_core::linter::RuleViolation::new(
            &quickmark_core::rules::md013::MD013,
            "Test default".to_string(),
            std::path::PathBuf::from("/test/file.md"),
            quickmark_core::linter::Range {
                start: quickmark_core::linter::CharPosition {
                    line: 1,
                    character: 0,
                },
                end: quickmark_core::linter::CharPosition {
                    line: 1,
                    character: 20,
                },
            },
        );

        let diagnostic = test_violation_to_diagnostic_with_config(&config, violation);

        // Should default to WARNING when rule not found in config
        assert_eq!(diagnostic.severity, Some(DiagnosticSeverity::WARNING));
    }

    #[test]
    fn test_violation_to_diagnostic_off_severity() {
        let config = create_test_config_with_severity("line-length", RuleSeverity::Off);

        let violation = quickmark_core::linter::RuleViolation::new(
            &quickmark_core::rules::md013::MD013,
            "Test off".to_string(),
            std::path::PathBuf::from("/test/file.md"),
            quickmark_core::linter::Range {
                start: quickmark_core::linter::CharPosition {
                    line: 0,
                    character: 0,
                },
                end: quickmark_core::linter::CharPosition {
                    line: 0,
                    character: 1,
                },
            },
        );

        let diagnostic = test_violation_to_diagnostic_with_config(&config, violation);

        // Off rules should be mapped to HINT (though they shouldn't normally reach this point)
        assert_eq!(diagnostic.severity, Some(DiagnosticSeverity::HINT));
    }

    #[test]
    fn test_diagnostic_range_mapping() {
        let config = QuickmarkConfig::default();

        // Test that ranges are correctly mapped from 0-based linter to 0-based LSP
        let violation = quickmark_core::linter::RuleViolation::new(
            &quickmark_core::rules::md001::MD001,
            "Heading levels should only increment by one level at a time".to_string(),
            std::path::PathBuf::from("/test/file.md"),
            quickmark_core::linter::Range {
                start: quickmark_core::linter::CharPosition {
                    line: 3,
                    character: 2,
                },
                end: quickmark_core::linter::CharPosition {
                    line: 3,
                    character: 12,
                },
            },
        );

        let diagnostic = test_violation_to_diagnostic_with_config(&config, violation);

        assert_eq!(diagnostic.range.start.line, 3);
        assert_eq!(diagnostic.range.start.character, 2);
        assert_eq!(diagnostic.range.end.line, 3);
        assert_eq!(diagnostic.range.end.character, 12);
    }

    #[test]
    fn test_lint_document_integration() {
        // Test the actual linting logic by using the lint functions directly
        use quickmark_core::config::config_in_path_or_default;
        use quickmark_core::linter::MultiRuleLinter;
        use std::env;

        let pwd = env::current_dir().unwrap();
        let config = config_in_path_or_default(&pwd).unwrap();

        // Test content with MD013 violations
        let content = "# This is a test heading that is way too long and exceeds the default 80 character limit";
        let file_path = std::path::PathBuf::from("/tmp/test.md");

        let mut linter = MultiRuleLinter::new_for_document(file_path, config.clone(), content);
        let violations = linter.analyze();

        assert!(
            !violations.is_empty(),
            "Should have violations for long line"
        );

        // Convert to diagnostics
        let diagnostics: Vec<Diagnostic> = violations
            .into_iter()
            .map(|violation| test_violation_to_diagnostic_with_config(&config, violation))
            .collect();

        // Should have MD013 violations
        let md013_violations: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.code == Some(NumberOrString::String("line-length".to_string())))
            .collect();
        assert!(
            !md013_violations.is_empty(),
            "Should have line-length violations"
        );
    }

    #[test]
    fn test_severity_mapping_comprehensive() {
        // Test all severity levels
        let severities = [
            (RuleSeverity::Error, DiagnosticSeverity::ERROR),
            (RuleSeverity::Warning, DiagnosticSeverity::WARNING),
            (RuleSeverity::Off, DiagnosticSeverity::HINT),
        ];

        for (rule_severity, expected_diagnostic_severity) in severities {
            let config = create_test_config_with_severity("line-length", rule_severity);

            let violation = quickmark_core::linter::RuleViolation::new(
                &quickmark_core::rules::md013::MD013,
                "Test message".to_string(),
                std::path::PathBuf::from("/test/file.md"),
                quickmark_core::linter::Range {
                    start: quickmark_core::linter::CharPosition {
                        line: 0,
                        character: 0,
                    },
                    end: quickmark_core::linter::CharPosition {
                        line: 0,
                        character: 1,
                    },
                },
            );

            let diagnostic = test_violation_to_diagnostic_with_config(&config, violation);
            assert_eq!(diagnostic.severity, Some(expected_diagnostic_severity));
        }
    }
}
