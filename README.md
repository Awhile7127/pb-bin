# pb-bin


## Description

A simple command line utility to send GET and POST requests to [bins](https://github.com/WantGuns/bin), an alternative to Pastebin.

Written in Rust, so should be platform-independent.


## Compiling

Tested using cargo:

### Debian / Ubuntu

```
sudo apt install cargo
cargo build -r
./target/pb-bin
```


## Usage

```
pb --help        See help and available options
pb <base-url> <method> <file> <get-url>
```

- base-url [REQUIRED]: The URL of the server where requests are sent
- method [OPTIONAL]: GET or POST; defaults to GET
- file [OPTIONAL]: The path to the file to POST to the server
- get-url [OPTIONAL]: The URL of the paste to be fetched from the server

To POST a file to http://www.sbcv.co.uk:8820 (my self-hosted bin), you would use:

```
pb -b http://www.sbcv.co.uk:8820 -m POST -f ~/test.txt
```

This prints the URL of the paste to the terminal. For instance: http://www.sbcv.co.uk:8820/p/test.txt

To GET the same paste, you would use:

```
pb -b http://www.sbcv.co.uk:8820 -m GET -u http://www.sbcv.co.uk:8820/p/test.txt
```
