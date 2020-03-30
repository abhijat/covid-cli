# covid-cli

Shows current counts from the command line for a country.

Data is sourced from https://github.com/pomber/covid19

```
~/dev/rust/covid-cli  (master) 
 abhijat $ ./target/release/covid-cli --help
covid-cli 0.1.0

USAGE:
    covid-cli [OPTIONS] --country <country>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --country <country>            The country to show records for
    -n, --num-results <num-results>    The last N days worth of records to show [default: 10]
```


I *really* hope this tool is soon rendered useless :-(
