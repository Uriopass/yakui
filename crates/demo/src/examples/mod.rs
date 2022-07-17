use std::str::FromStr;

use anyhow::bail;
use clap::Parser;

use crate::ExampleState;

macro_rules! examples {
    ($macro:ident) => {
        $macro!(bench, sandbox, align, images, text, window, row_column, flex, inputs);
    };
}

/// Run a yakui example.
#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub example: Example,
}

macro_rules! define_example {
    ($($mod:ident),* $(,)?) => {
        $(pub mod $mod;)*

        #[derive(Debug, Parser)]
        #[allow(non_camel_case_types)]
        pub enum Example {
            $(
                #[clap(about = concat!("example: ", stringify!($mod)))]
                $mod,
            )*
        }

        impl Example {
            pub fn function(&self) -> &'static dyn Fn(&ExampleState) {
                match self {
                    $(Example::$mod => &$mod::run,)*
                }
            }
        }

        impl FromStr for Example {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($mod) => Ok(Self::$mod),)*
                    unknown => {
                        let example_list = [$(stringify!($mod),)*].join(", ");
                        bail!("unknown example '{unknown}', included examples are: {example_list}");
                    },
                }
            }
        }
    }
}

examples!(define_example);