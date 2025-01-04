# Cedarical

This is a tool for prettifying a Cedarville schedule iCalendar file (retrieved from [this page](https://selfservice.cedarville.edu/cedarinfo/info?schedule=1) with the download button).
Highly opinionated.

The one issue you might run into is the mappings for buildings (I was only able to find values for buildings I've had classes in so far).
If you would like to add an acronym, just open an issue (or PR if you feel like it), and make sure to tell me the original building name from the ICS file.

## Installation & Usage

To install, clone the repository to your local machine and install with the following command:
```sh 
cargo install --path .
```

Then, usage should be as easy as:
```sh 
cedarical input_schedule.ics
```

See `cedarical --help` for more options.
