# GURPS ChGen, a.k.a. "chgen"
**GURPS ChGen** &mdash; a spiritual kin of *Bill Seurer's* MS-DOS **MakeChar**.

# DTA/GEN Files of MakeChar
**chgen** reads **MakeChar** DTA/GEN files natively.

# #XCG/DATA Files of GURPS ChGen
An early variant of the above mentioned DTA/GEN format. Not per se supported.

# JSON
Well, this is the main format to use.

**chgen** comes with a small tool to convert DTA/GEN files into JSON, called **dta2json**.

## DTA/GEN → JSON
### Batch conversion
`dta2json`

**dta2json** will scan the neighborhood for a directory called `datafiles`, which should
house all the DTA/GEN files you want to convert at once.

### Single-file conversion
`dta2json file.dta > file.chgen`

The command above converts a single file into **chgen**'s JSON format.
