# MD003 Setext with ATX Style Violations

This file demonstrates violations of setext_with_atx and setext_with_atx_closed configurations.

# Level 1 ATX - Should be setext in setext_with_atx config

## Level 2 ATX - Should be setext in setext_with_atx config  

### Level 3 ATX Closed - Should be open ATX in setext_with_atx config ###

#### Level 4 ATX Closed - Should be open ATX in setext_with_atx config ####

# Another Level 1 ATX - Should be setext

## Another Level 2 ATX - Should be setext

### Another Level 3 ATX Open - Correct for setext_with_atx

#### Another Level 4 ATX Open - Correct for setext_with_atx

##### Level 5 ATX Closed - Should be open ATX #####

###### Level 6 ATX Closed - Should be open ATX ######

Proper setext_with_atx should use:
- Setext for levels 1-2
- Open ATX for levels 3+