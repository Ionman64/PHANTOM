# RHINO: Repository History INformation Obtainer
Repository for RHINO developed as part of the **masters thesis** 

> Characterising Software Development Projects based on git commit data and static code analysis -- A case study of open source GitHub projects

of **Joshua Jungen and Peter Pickerill** at **Chalmers University of Technology**.

# Project Setup Guide
The following steps are tested on Ubuntu 17.10. To correctly setup the project follow this guide step by step.

## Package dependencies
The following packages are required to setup the project. 

    git
	make
	curl
	clang


## Install rust
Run the following command to install rust. Follow the installation guide and choose the default installation.

	$ curl https://sh.rustup.rs -sSf | sh

## Add *cargo* to current shell
Run the following command to make use of *cargo* in the **current shell**. 

	$ source ~/.cargo/env 

(For a permanent use of cargo in the command line make sure to add *~/.cargo/bin* to your path.)

## Clone and configure the project
Next, clone the project to your machine and change your directory to the top folder.
	
	$ git clone https://github.com/Ionman64/ProjectAnalyser.git
	$ cd ProjectAnalyser

# Run the test suite
To make sure the project is setup correctly run the following script.

    $ ./scripts/test_runner.sh

**Note:** The script runs all tests for the project. Some tests need to be run single-threaded (Such as the database), which is why using *cargo test* would produces unreliable test results. The script invokes *cargo test* with the required flags to make sure that tests are executed correctly.
