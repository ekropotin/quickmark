# MD019 Comprehensive Test - Multiple Spaces After Hash

This file contains a comprehensive mix of valid and invalid examples for MD019 testing.

## Valid Examples (No Violations)

# Level 1 heading (valid)
## Level 2 heading (valid)
### Level 3 heading (valid)
#### Level 4 heading (valid)
##### Level 5 heading (valid)
###### Level 6 heading (valid)

#No space heading (valid - not MD019's concern)
##No space heading (valid - not MD019's concern)

# Closed heading with single space #
## Closed heading with single space ##
### Closed heading with single space ###

## Invalid Examples (Should Trigger Violations)

##  Two spaces violation

###   Three spaces violation

####    Four spaces violation

#####     Five spaces violation

######      Six spaces violation

##	Single tab violation

###		Two tabs violation

####  	Mixed space and tab violation

#####	 Tab then space violation

######   	Multiple chars violation

## Closed ATX with Violations

##  Closed with two spaces ##

###   Closed with three spaces ###

####    Closed with four spaces ####

## Mixed Valid and Invalid

# Valid level 1
##  Invalid level 2 (two spaces)
### Valid level 3
####   Invalid level 4 (three spaces)
##### Valid level 5
######    Invalid level 6 (four spaces)

## Edge Cases

### 	Single tab (should violate)

#### 	 Tab and space (should violate)

#####  Single extra space (should violate)

##  Violation at start of line

	###   Indented heading with violation

## Complex Content with Violations

##  Heading with `code` spans

###   Heading with *emphasis* and **bold**

####    Heading with [links](http://example.com)

#####     Heading with emoji 🎉 and numbers

######      Heading with special chars !@#$%

## Valid Complex Content

# Heading with `code` spans (valid)

## Heading with *emphasis* and **bold** (valid)

### Heading with [links](http://example.com) (valid)

#### Heading with emoji 🎉 and numbers (valid)

##### Heading with special chars !@#$ (valid)