use cyclonedds_idlc::{generate_with_loader, Configuration, IdlLoader};
use getopts::Options;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone, Default)]
struct Loader {
    search_path: Vec<String>,
}

fn load_from(prefix: &std::path::Path, filename: &str) -> Result<String, Error> {
    let fullname = prefix.join(filename);

    let mut file = File::open(fullname)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    return Ok(data);
}

impl Loader {
    pub fn new(search_path: Vec<String>) -> Loader {
        Loader {
            search_path: search_path,
        }
    }
}

impl IdlLoader for Loader {
    fn load(&self, filename: &str) -> Result<String, Error> {
        for prefix in &self.search_path {
            let prefix_path = std::path::Path::new(&prefix);
            match load_from(&prefix_path, filename) {
                Ok(data) => return Ok(data),
                _ => continue,
            }
        }
        Err(Error::from(ErrorKind::NotFound))
    }
}

//
fn print_usage(program: &str, opts: Options) -> Result<(), std::io::Error> {
    let brief = format!(
        "Usage: {} [-o <outfile>] [-I <include_dir>] <idlfile>",
        program
    );
    print!("{}", opts.usage(&brief));
    return Err(Error::from(ErrorKind::NotFound));
}

fn main() -> Result<(), std::io::Error> {
    let mut opts = Options::new();
    opts.optmulti(
        "I",
        "",
        "Add the specified 'directory' to the search path for include files.",
        "directory",
    );
    opts.optmulti(
        "D",
        "",
        "Predefine 'name' as a macro, with definition 1.",
        "name",
    );
    opts.optopt("o", "", "Write output to 'outfile'.", "outfile");
    opts.optflag("v", "", "Verbose output for debugging'.");
    opts.optflag("h", "help", "print this help menu");
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        return print_usage(&program, opts);
    }

    let search_path = matches.opt_strs("I");

    let defs = matches
        .opt_strs("D")
        .into_iter()
        .map(|d| {
            let mut iter = d.splitn(2, "=");
            match (iter.next(), iter.next()) {
                (Some(key), None) => (key.to_owned(), "1".to_owned()),
                (Some(key), Some(val)) => (key.to_owned(), val.to_owned()),
                (None, _) => panic!(),
            }
        })
        .collect::<HashMap<String, String>>();

    let mut loader = Loader::new(search_path);

    let infile = match matches.free.len() {
        1 => matches.free[0].clone(),
        _ => return print_usage(&program, opts),
    };

    let data = load_from(&env::current_dir().unwrap(), &infile)
        .map_err(|_| Error::new(ErrorKind::NotFound, ""))?;

    let config = Configuration::new(defs, matches.opt_present("v"));

    let result = match matches.opt_str("o") {
        Some(outfile) => {
            let mut of = File::create(std::path::Path::new(&outfile))?;
            generate_with_loader(&mut of, &mut loader, &config, &data)
        }
        _ => generate_with_loader(&mut io::stdout(), &mut loader, &config, &data),
    };

    match result {
        Ok(_) => Ok(()),
        Err(err) => {
            eprint!("parse error {:?}", err);
            Err(Error::new(ErrorKind::InvalidData, "parse error"))
        }
    }
}

/*


//type IdlPair<'a> = pest::iterators::pairs::Pairs<<'a>rtps_idl_grammar::Rule>;

fn main() {
    println!("Hello, world!");

    let mut file = std::fs::File::open("/home/sjames/buildenv/work/cyclonedds/examples/helloworld/HelloWorldData.idl").unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let i = IdlParser::parse(Rule::specification, &data).expect("Unable to parse").next().unwrap();
    //for p in i {
        //println!("{:?} ",i);
    //}

    for record in i.into_inner() {
        match record.as_rule() {
            Rule::definition => {
                //println!("Definition:{:?}", record.into_inner());
                let it = record.into_inner();
                print_type_of(&it);
            },
            _=> {}
        }
    }

}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn definition(pair : &pest::iterators::pairs::Pairs<rtps_idl_grammar::Rule>) {

}
*/
