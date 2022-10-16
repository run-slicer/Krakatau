use std::path::PathBuf;

use clap::Parser;

use crate::file_input_util;
use crate::file_output_util::Writer;
use crate::lib::disassemble;
use crate::lib::DisassemblerOptions;
use crate::lib::ParserOptions;

#[derive(Parser)]
pub struct DisassemblerCli {
    input: PathBuf,
    #[clap(short, long, parse(from_os_str))]
    out: PathBuf,

    #[clap(short, long)]
    roundtrip: bool,

    #[clap(long)]
    no_short_code_attr: bool,
}

pub fn disassembler_main(cli: DisassemblerCli) -> i32 {
    let opts = DisassemblerOptions {
        roundtrip: cli.roundtrip,
    };
    let parse_opts = ParserOptions {
        no_short_code_attr: cli.no_short_code_attr,
    };

    let mut writer = Writer::new(&cli.out);
    let mut error_count = 0;
    file_input_util::read_files(&cli.input, "class", |fname, data| {
        println!("disassemble {}", fname);
        let (name, out) = match disassemble(&data, parse_opts, opts) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("Parse error in {}: {}", fname, err.0);
                error_count += 1;
                return;
            }
        };
        let name = name.map(|name| format!("{}.j", name));
        writer.write(name.as_deref(), &out);
        println!("Wrote {} bytes to {}", out.len(), name.as_deref().unwrap_or("file"));
    });

    // set exit code 1 if there were errors
    if error_count > 0 {
        1
    } else {
        0
    }
}
