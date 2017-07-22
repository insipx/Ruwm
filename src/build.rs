extern crate peg;

/**
 * @author Andrew Plaza
 * Builds the grammar for configuration files
 *
 */

fn main() {
    peg::cargo_build("src/config_parser/config_grammar.rustpeg");
}


