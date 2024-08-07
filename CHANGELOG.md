# CHANGE LOG

## Current changes without release yet

- Repaired error #91 adding a force-overwrite option to the command line

## [0.1.6] - 2024-08-09

- Added more features to SHACL validation: #94
- Added more control about syntax highlighting on terminal:
  - Avoiding to include colors when the output goes to a file in ShEx generation options
- Added config parameter to some of the options in the Command line tool so the user can configure the behaviour: validate, convert, dctap, node

## [0.1.5] - 2024-07-30

- Added options in command line to pass config files in YAML
- Repaired bug in DCTAP resolution of IRIs

## [0.1.4] - 2024-07-28

- Added 2 separate options for shacl-validate and shex-validate, keeping the generic validate option
- Repaired bug on UML visualization that didn't show link names
- Added direct SVG/JPG generation from DCTAP files

## [0.1.3] - 2024-07-27

- Generation of HTML views from ShEx based on Minininja templates which allow better customization
- Direct conversion from DCTAP to UML and HTML views
- Generation of UML visualizations in SVG and PNG
- Basic support for SHACL validation and added shacl-validation crate

## [0.1.2] - 2024-07-17

- Added descriptions to subcommands in command line
- Added more options in DCTAP: property and shape labels, and value constraints
- Added direct conversion from DCTAP to HTML and UML
- More options for HTML views

## [0.1.1] - 2024-07-12

- Added basic support for generating HTML views from ShEx schemas, #60

## [0.1.0] - 2024-07-05

- Added fields: mandatory, repeatable, valueDatatype and valueShape to DCTAP
- Repaired spelling errors in README issue #73

## [0.0.15] - 2024-07-04

- First version with support for conversion from ShEx schemas to UML

## [0.0.14] - 2024-07-02

- First version with initial support for DCTap to ShEx converter, issue #54
- Refactor on shapes converter to accomodate more conversions each of them in its own folder
- First version which publishes also Python bindings

## [0.0.13] - 2024-06-22

- First version with initial support for ShEx to SPARQL converter, issue #67

## [0.0.12] - 2024-06-17

- Changed CLI name from `sx` to `rdfsx`
- First attempt to added basic support for DCTap
- Code cleaned with Rustfmt and Clippy by [MarcAntoine-Arnaud](https://github.com/MarcAntoine-Arnaud).

## [0.0.11] - 2024-06-08

- This version in mainly a maintainance version updating some dependencies
- Started project DCTAP to handle DCTAP files
- Updated some dependency versions
  - oxrdf = "0.2.0-alpha.2"
  - regex = "1.10.4"

## [0.0.10] - 2024-01-29

- [issue 32](https://github.com/weso/shapes-rs/issues/32) ShEx parser works as an iterator per statement allowing to show debug information by statement. Debug information can be controlled by the environment variablt RUST_LOG. A value of "debug" for that variable will print more information.
- Updated dependency versions
    oxrdf = "0.2.0-alpha.2"
    oxttl = "0.1.0-alpha.2"
    oxrdfio = "0.1.0-alpha.2"

## [0.0.9] - 2024-01-19

- Removed `shex_pest`, `shex_antlr` and `validation_oxgraph` folders because their code is no longer used.
- Added time option to `sx_cli`
- Repaired bug in `shex_compact` that failed with node constraints followed by cardinality without space
- More support to read SHACL as RDF
- Merged [srdf_graph](https://crates.io/crates/srdf_graph) and [srdf_sparql](https://crates.io/crates/srdf_sparql) crates into [srdf](https://crates.io/crates/srdf), the former crates will no longer be maintained as their code is integrated in `srdf`.
- Added option `--output` to CLI so the users can choose if the output goes to terminal or to a file
- Changed dependency from [rio_api](https://crates.io/crates/rio) and [rio_turtle](https://crates.io/crates/rio_turtle) to [oxttl](https://crates.io/crates/oxttl) and [oxrdfio](https://crates.io/crates/oxrdfio) which seem to be more actively maintained now.

## [0.0.7] - 2024-01-07

In this release we added support for SHACL by defining the [`shacl_ast`](https://crates.io/crates/shacl_ast) crate.

Other changes:

- Renamed the project from shex_rs to shapes_rs to indicate that the project intends to support both ShEx and SHACL.
- Merged the [srdf_graph](https://crates.io/crates/srdf_graph) and [srdf_sparql](https://crates.io/crates/srdf_sparql) crates into [srdf](https://crates.io/crates/srdf).
- Added more combinators and documentation examples to rdf_parser in order to document the RDF parser combinators approach. See, for example, the doc for the [map method](https://docs.rs/srdf/latest/srdf/srdf_parser/trait.RDFNodeParse.html#method.map).
