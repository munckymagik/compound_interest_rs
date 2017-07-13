#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches};

fn compound(principal: f64, annual_rate_percent: f64, period_in_years: i32, compounds_per_year: i32) -> f64 {
    let num_compounds = compounds_per_year * period_in_years;

    let annual_rate_fraction = annual_rate_percent / 100.0;
    let r = 1.0 + (annual_rate_fraction / compounds_per_year as f64);

    let mut p = principal;
    for _ in 0..num_compounds {
        p *= r;
    }

    p
}

fn compound_annually(principal: f64, annual_rate_percent: f64, period_in_years: i32) -> f64 {
    compound(principal, annual_rate_percent, period_in_years, 1)
}

fn compound_monthly(principal: f64, annual_rate_percent: f64, period_in_years: i32) -> f64 {
    compound(principal, annual_rate_percent, period_in_years, 12)
}

fn compound_daily(principal: f64, annual_rate_percent: f64, period_in_years: i32) -> f64 {
    compound(principal, annual_rate_percent, period_in_years, 365)
}

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .about("Calculates compound interest")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .arg(Arg::with_name("principal").required(true))
        .arg(Arg::with_name("annual_rate_percent").required(true))
        .arg(Arg::with_name("period_in_years").required(true))
        .get_matches()
}

fn main() {
    let args = parse_args();

    let principal = args.value_of("principal").unwrap().parse().unwrap();
    let annual_rate_percent = args.value_of("annual_rate_percent").unwrap().parse().unwrap();
    let period_in_years = args.value_of("period_in_years").unwrap().parse().unwrap();

    println!("£{:.3} Anually", compound_annually(principal, annual_rate_percent, period_in_years));
    println!("£{:.3} Monthly", compound_monthly(principal, annual_rate_percent, period_in_years));
    println!("£{:.3} Daily", compound_daily(principal, annual_rate_percent, period_in_years));
}
