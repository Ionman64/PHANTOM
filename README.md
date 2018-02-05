# ProjectAnalyser
Repository for the project analyser developed as part of the **masters thesis** 

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
	postgresql-9.6 
	postgresql-contrib-9.6 
	postgresql-server-dev-9.6 

## Setup database user and password
After successfully installing the PostgreSql related packages the database user has to be configured. By default, the user name is *postgres* and no password is set. Set a password by running the folling commands.

	$ sudo -i -u postgres
	$ psql
	$ \password


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

The project requires several environment variables. These are stored in the *.env* file which has to be setup manually as it contains information specific to your setup. The file has to be located at the root directory. At the time of writing (3rd February, 2018), it needs to include the following variables:
	
	DATABASE_URL=postgres://postgres:<password>@localhost/project_analyser
	
	TESTDATABASE_URL=postgres://postgres:<password>@localhost/test_pa
	
	DBSERVER_URL=postgres://postgres:<password>@localhost

**Note:** The user name (postgres) and the database names (project_analyser, test_pa) might change in the future. The password for the user has to replace <password>.

## Setup the project database
The *diesel client* is a command line tool to manage database schemas (i.e. used to generate and run database migrations). **Make sure you have installed all dependencies and setup the database server and user, and created the environment variable file as described above.** Install it by running the following command from the project's root directory. 

	$ cargo install diesel_cli --no-default-features --features "postgres"

Now, *diesel* can be used from a command line. This creates the database and runs all upward migrations.

	$ diesel setup

# Run the test suite
To make sure the project is setup correctly run the following script.

    $ ./scripts/test_runner.sh

**Note:** The script runs all tests for the project. Some tests need to be run single-threaded (Such as the database), which is why using *cargo test* would produces unreliable test results. The script invokes *cargo test* with the required flags to make sure that tests are executed correctly.
