// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod graph;
use graph::{Edge, EdgeIndex, Graph, Node, NodeIndex};
use regex::Regex;
use std::env;

#[derive(Debug)]
pub enum ArgType<'a> {
    Flag(Flag<'a>),
    SubCommand(SubCommand<'a>),
    Argument(&'a str),
    Unknown(&'a str),
    UnknownFlag(&'a str),
    Over,
}

#[derive(Debug)]
pub struct Arg<'a> {
    kind: ArgType<'a>,
    found: bool,
}

impl<'a> Arg<'a> {
    fn new(arg_type: ArgType<'a>) -> Self {
        Arg {
            kind: arg_type,
            found: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Flag<'a> {
    name: &'a str,
    short: char,
    long: &'a str,
    takes_arg: bool,
}

impl<'a> Flag<'a> {
    fn new(name: &'a str, short: char, long: &'a str, takes_arg: bool) -> Self {
        Flag {
            name,
            short,
            long,
            takes_arg,
        }
    }
}

#[derive(Debug)]
pub struct SubCommand<'a> {
    name: &'a str,
    aliases: Vec<&'a str>,
}

impl<'a> SubCommand<'a> {
    fn new(name: &'a str, aliases: Vec<&'a str>) -> Self {
        SubCommand { name, aliases }
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    graph: Graph<Arg<'a>>,
    binary_flags: Vec<Flag<'a>>,
    subcommands: Vec<SubCommandConfig<'a>>,
    current_subcmd: Option<NodeIndex>,
}

impl<'a> Parser<'a> {
    fn new() -> Self {
        let graph = Graph::<Arg>::new();
        Parser {
            graph,
            binary_flags: vec![],
            subcommands: vec![],
            current_subcmd: None,
        }
    }

    pub fn flag(
        &mut self,
        name: &'a str,
        short: char,
        long: &'a str,
        takes_arg: bool,
    ) -> &mut Self {
        self.binary_flags
            .push(Flag::new(name, short, long, takes_arg));
        self
    }

    pub fn help(&mut self) -> &mut Self {
        self.binary_flags
            .push(Flag::new("help", 'h', "help", false));
        self
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.binary_flags
            .push(Flag::new("verbose", 'V', "verbose", false));
        self
    }

    pub fn version(&mut self) -> &mut Self {
        self.binary_flags
            .push(Flag::new("version", 'v', "version", false));
        self
    }

    pub fn license(&mut self) -> &mut Self {
        self.binary_flags
            .push(Flag::new("license", 'L', "license", false));
        self
    }

    pub fn debug(&mut self) -> &mut Self {
        self.binary_flags
            .push(Flag::new("debug", 'd', "debug", false));
        self
    }

    pub fn subcommand(&mut self, subcommand: SubCommandConfig<'a>) -> &mut Self {
        if let Some(_) = self
            .subcommands
            .iter()
            .find(|&subcmd| subcmd.name == subcommand.name)
        {
            panic!("cannot have two subcommands with the same name at the same level");
        }
        self.subcommands.push(subcommand);
        self
    }

    pub fn tap(&mut self, args: Vec<&'a str>) -> &mut Self {
        self.build_graph();
        println!("graph builded");
        self.iterate_args(args);
        println!("{:#?}", self.graph);
        self
    }

    fn build_graph(&mut self) -> &mut Self {
        for flag in &self.binary_flags {
            self.graph.add_node(Arg::new(ArgType::Flag(*flag)));
        }
        for subcommand in &self.subcommands {
            iterate_subcommand_config(&mut self.graph, &subcommand, None);
        }
        self
    }

    fn iterate_args(&mut self, args: Vec<&'a str>) {
        let mut accept_opt = true;
        while let Some(&arg) = args.iter().next() {
            if arg == "-" {
                // self.graph.add_node(ArgType::Argument(arg));
            } else if arg == "--" {
                // self.graph.add_node(ArgType::Over);
                accept_opt = false;
            } else if arg.len() > 2 && arg.starts_with("--") && accept_opt == true {
                // self.parse_long_option(&arg);
            } else if arg.len() > 1 && arg.starts_with("-") && accept_opt == true {
                // self.parse_option(&arg);
            } else {
                if !self.handle_subcommand(arg) {
                    let node_index;
                    let data = Arg::new(ArgType::Argument(arg));
                    if let Some(index) = self.current_subcmd {
                        node_index = self.graph.add_node_to(index, data);
                    } else {
                        node_index = self.graph.add_node(data);
                    }
                    self.graph.nodes[node_index.0].data.found = true;
                }
            }
        }
    }

    fn handle_subcommand(&mut self, arg: &str) -> bool {
        true
        // let direct_children = self.graph.children(self.current_subcmd);
        // let result = direct_children.find(|index| {
        // if let ArgType::SubCommand(subcommand) = &self.graph.nodes[index.0].data.kind {
        // if subcommand.name == arg {
        // return true;
        // }
        // if let Some(_) = subcommand.aliases.iter().find(|&&alias| alias == arg) {
        // return true;
        // }
        // }
        // false
        // });
        // if let Some(&index) = result {
        // self.graph.nodes[index.0].data.found = true;
        // self.current_subcmd = Some(index);
        // return true;
        // }
        // false
    }

    // fn parse_long_option(&mut self, arg: &str) {
    // let current_arg = &arg[2..];
    // let direct_children = self.graph.direct_children(self.current_subcmd);
    // match current_arg.find("=") {
    // None => {
    // if let Some(i) = direct_children.iter().find(|index| {
    // if let ArgType::Flag(flag) = &self.graph.nodes[index.0].data.kind {
    // if flag.long == current_arg {
    // return true;
    // }
    // }
    // false
    // }) {
    // self.graph.nodes[i.0].data.found = true;
    // } else {
    // let index = self
    // .graph
    // .add_node(Arg::new(ArgType::UnknownFlag(current_arg)));
    // if let Some(i) = self.current_subcmd {
    // self.graph.add_edge(i, index);
    // }
    // }
    // }
    // Some(i) => {
    // let first = &current_arg[..i];
    // let last = &current_arg[i + 1..];
    // if let Some(i) = direct_children.iter().find(|index| {
    // if let ArgType::Flag(flag) = &self.graph.nodes[index.0].data.kind {
    // if flag.long == first {
    // return true;
    // }
    // }
    // false
    // }) {
    // // if option.3 == true && !last.is_empty() {
    // // tokens.push(Token::Option(&option, Some(String::from(last))));
    // // } else {
    // // tokens.push(Token::Option(&option, None));
    // // }
    // } else {
    // let index = self
    // .graph
    // .add_node(Arg::new(ArgType::UnknownFlag(current_arg)));
    // if let Some(i) = self.current_subcmd {
    // self.graph.add_edge(i, index);
    // }
    // }
    // }
    // }
    // }
}

fn iterate_subcommand_config<'a>(
    graph: &mut Graph<Arg<'a>>,
    current_subcmd: &SubCommandConfig<'a>,
    previous_index: Option<NodeIndex>,
) {
    let subcmd_index;
    let data = Arg::new(ArgType::SubCommand(SubCommand::from(current_subcmd)));
    if let Some(index) = previous_index {
        subcmd_index = graph.add_node_to(index, data);
    } else {
        subcmd_index = graph.add_node(data);
    }
    for flag in &current_subcmd.flags {
        graph.add_node_to(subcmd_index, Arg::new(ArgType::Flag(*flag)));
    }
    for subcommand in &current_subcmd.subcommands {
        iterate_subcommand_config(graph, subcommand, Some(subcmd_index));
    }
}

#[derive(Debug)]
pub struct SubCommandConfig<'a> {
    flags: Vec<Flag<'a>>,
    name: &'a str,
    aliases: Vec<&'a str>,
    subcommands: Vec<SubCommandConfig<'a>>,
}

impl<'a> SubCommandConfig<'a> {
    fn with_name(name: &'a str) -> Self {
        if name.is_empty() || Regex::new(r"\W").unwrap().is_match(name) {
            panic!("a subcommand must be defined with a valid name");
        }
        SubCommandConfig {
            flags: vec![],
            name: name,
            subcommands: vec![],
            aliases: vec![],
        }
    }

    pub fn alias(mut self, alias: &'a str) -> Self {
        self.aliases.push(alias);
        self
    }

    pub fn flag(mut self, name: &'a str, short: char, long: &'a str, takes_arg: bool) -> Self {
        self.flags.push(Flag::new(name, short, long, takes_arg));
        self
    }

    pub fn help(mut self) -> Self {
        self.flags.push(Flag::new("help", 'h', "help", false));
        self
    }

    pub fn verbose(mut self) -> Self {
        self.flags.push(Flag::new("verbose", 'V', "verbose", false));
        self
    }

    pub fn version(mut self) -> Self {
        self.flags.push(Flag::new("version", 'v', "version", false));
        self
    }

    pub fn debug(mut self) -> Self {
        self.flags.push(Flag::new("debug", 'd', "debug", false));
        self
    }

    pub fn subcommand(mut self, subcommand: SubCommandConfig<'a>) -> Self {
        if let Some(_) = self
            .subcommands
            .iter()
            .find(|&subcmd| subcmd.name == subcommand.name)
        {
            panic!("cannot have two subcommands with the same name at the same level");
        }
        self.subcommands.push(subcommand);
        self
    }
}

impl<'a> From<&SubCommandConfig<'a>> for SubCommand<'a> {
    fn from(subcmd: &SubCommandConfig<'a>) -> Self {
        SubCommand {
            name: subcmd.name,
            aliases: subcmd.aliases.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_new() {
        let mut parser = Parser::new();
        // assert_eq!(parser.binary_flags.len(), 0);
        parser.help();
        // assert_eq!(parser.binary_flags.len(), 1);
        // parser.license();
        // assert_eq!(parser.binary_flags.len(), 2);
        // parser.subcommand(SubCommandConfig::with_name("test").help().verbose());
        parser.subcommand(
            SubCommandConfig::with_name("binary_subcmd")
                .verbose()
                .alias("bin")
                .subcommand(SubCommandConfig::with_name("subsubcmd").debug()),
        );
        let args = vec!["tap", "test", "-hvd", "--help"];
        // parser.tap(args);
        // let args = vec!["tap", "test", "-hvd", "--help"];
        // let parsed = parser.tap_from(args);
        // assert_eq!(parsed.args.len(), 1);
        // assert_eq!(parsed.options.len(), 2);
        // assert_eq!(parsed.scraps.len(), 2);
    }
}
// pub struct Parsed {}
//
// impl<'a> Parser<'a> {
// pub fn new() -> Parser<'a> {
// let mut graph = Graph::<ArgType>::new();
// graph.add_node(ArgType::Root);
// let levels = vec![];
// levels.push(Level::newBinary());
// Parser {
// graph,
// levels,
// current_level: Some(NodeIndex(0)),
// }
// }
//
// pub fn flag(
// &mut self,
// name: &'a str,
// short: char,
// long: &'a str,
// takes_value: bool,
// ) -> &mut Parser<'a> {
// self.flags.push(Flag(name, short, long, takes_value));
// self
// }
//
// pub fn help(&mut self) -> &mut Parser<'a> {
// self.flags.push(Flag("help", 'h', "help", false));
// self
// }
//
// pub fn verbose(&mut self) -> &mut Parser<'a> {
// self.flags.push(Flag("verbose", 'V', "verbose", false));
// self
// }
//
// pub fn version(&mut self) -> &mut Parser<'a> {
// self.flags.push(Flag("version", 'v', "version", false));
// self
// }
//
// pub fn license(&mut self) -> &mut Parser<'a> {
// self.flags.push(Flag("license", 'L', "license", false));
// self
// }
//
// pub fn debug(&mut self) -> &mut Parser<'a> {
// self.flags.push(Flag("debug", 'd', "debug", false));
// self
// }
//
// pub fn tap(&mut self) -> Parsed {
// let mut args = env::args();
// args.next();
// // self.logic(args)
// let mut parsed = Parsed {};
// parsed
// }
//
// pub fn tap_from(&mut self, mut args: Vec<&str>) -> Parsed {
// let mut iter = args.iter().map(|arg| arg.to_string());
// iter.next();
// // self.logic(iter)
// let mut parsed = Parsed {};
// parsed
// }

// fn logic<A: Iterator<Item = String>>(&mut self, mut args: A) -> Parsed {
// let mut parsed = Parsed {};
// let mut tokens = self.tokenize(&mut args);
// parsed
// }
//
// fn tokenize<A: Iterator<Item = String>>(&mut self, mut args: A) {
// let mut accept_opt = true;
// while let Some(arg) = args.next() {
// if arg == "-" {
// // tokens.push(Token::Argument(String::from("-")));
// self.graph.add_node(ArgType::Argument(arg));
// } else if arg == "--" {
// self.graph.add_node(ArgType::Over);
// accept_opt = false;
// } else if arg.len() > 2 && arg.starts_with("--") && accept_opt == true {
// self.parse_long_option(&arg);
// } else if arg.len() > 1 && arg.starts_with("-") && accept_opt == true {
// self.parse_option(&arg);
// } else {
// // tokens.push(Token::Argument(String::from(arg)));
// }
// }
// }

// fn normalize<'a>(&mut self, tokens: &mut Vec<Token<'a>>) {
// let mut to_merge = vec![];
// let mut inc = 0;
// let mut token_iter = tokens.iter().enumerate().peekable();
// while let Some((i, token)) = token_iter.next() {
// if let Token::Option(flag, arg) = token {
// if flag.3 == true && *arg == None {
// if let Some((_j, Token::Argument(value))) = token_iter.peek() {
// to_merge.push((i - inc, *flag, value.to_string()));
// inc += 1;
// }
// }
// }
// }
// for (i, flag, arg) in to_merge {
// tokens.remove(i);
// tokens.remove(i);
// tokens.insert(i, Token::Option(flag, Some(arg)));
// }
// }

// fn parse_option(&mut self, arg: &str) {
// let current_arg = &arg[1..];
// for (i, c) in current_arg.char_indices() {
// if let Some(option) = self.flags.iter().find(|option| c == option.1) {
// if option.3 == true {
// if i + 1 < current_arg.len() {
// let arg_opt = &current_arg[i + 1..];
// // tokens.push(Token::Option(&option, Some(String::from(arg_opt))));
// break;
// } else {
// // tokens.push(Token::Option(&option, None));
// }
// } else {
// // tokens.push(Token::Option(&option, None));
// }
// } else {
// // tokens.push(Token::UnknownOpt(c.to_string()));
// }
// }
// }

// fn parse_long_option(&mut self, arg: &str) {
// let current_arg = &arg[2..];
// match current_arg.find("=") {
// None => {
// if let Some(option) = self.flags.iter().find(|option| current_arg == option.2) {
// tokens.push(Token::Option(&option, None));
// } else {
// tokens.push(Token::UnknownOpt(current_arg.to_string()));
// }
// }
// Some(i) => {
// let first = &current_arg[..i];
// let last = &current_arg[i + 1..];
// if let Some(option) = self.flags.iter().find(|option| first == option.2) {
// if option.3 == true && !last.is_empty() {
// tokens.push(Token::Option(&option, Some(String::from(last))));
// } else {
// tokens.push(Token::Option(&option, None));
// }
// } else {
// tokens.push(Token::UnknownOpt(current_arg.to_string()));
// }
// }
// }
// }
// }

// #[derive(Debug)]
// enum Token<'a> {
// Argument(String),
// Option(&'a Flag, Option<String>),
// UnknownOpt(String),
// }

// #[derive(Debug)]
// pub struct Opt<'a> {
// flag: &'a Flag,
// arg: Option<String>,
// }

// impl<'a> Opt<'a> {
// fn new(flag: &'a Flag, arg: Option<String>) -> Opt {
// Opt { flag, arg }
// }
// }

// #[derive(Debug)]
// pub struct Parsed<'a> {
// options: Vec<Opt<'a>>,
// args: Vec<String>,
// scraps: Vec<String>,
// }
