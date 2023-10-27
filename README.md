# GOPM

### Golang Pocket Manager (And Templater)

## Installation

```bash
git clone https://github.com/golangpm/gopm
cd gopm
Make an enviroment variable from bin/gopm.exe
```

## Usage

### gopm command

```bash
   ____    ___    ____    __  __ 
  / ___|  / _ \  |  _ \  |  \/  |
 | |  _  | | | | | |_) | | |\/| |
 | |_| | | |_| | |  __/  | |  | |
  \____|  \___/  |_|     |_|  |_|


Invalid command. Use 'gopm --help' for usage information.
```

### gopm -h
```bash
gopm 0.0.4                                     
ITDOBRO                                        
A Go project manager and template generator    
                                               
USAGE:                                         
    gopm.exe [SUBCOMMAND]                      
                                               
FLAGS:                                         
    -h, --help       Prints help information   
    -V, --version    Prints version information

SUBCOMMANDS:
    build         Build the Go application
    get-author    Get the saved author
    help          Prints this message or the help of the given subcommand(s)
    init          Initialize the file structure of the current directory
    new           Create a new Go application with a custom file structure template
    run           Build and run the Go application
    set-author    Save the author to the configuration
                        Example: set-author Your Name
```

## Documentation

To load project documentation, please run this commands:

```bash
cargo rustdoc
cargo rustdoc --open
```

The Documentation will be open at the yous browser window
