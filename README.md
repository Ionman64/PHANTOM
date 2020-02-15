# PHANTOM Toolset
Repository for PHANTOM developed as part of the **masters thesis** 

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

# Configuration
There are several lines in the project in the main.rs file which allow for some configuration; 

	$ const THREAD_POOL_SIZE:usize = 3;
	$ const PROJECTS_FILE:&str = "projects.csv";
	$ const LOGS_FOLDER:&str = "/home/pa2/project_downloader/git_log";
	
## THREAD_POOL_SIZE 
Specifies how many projects should be downloaded at the same time (each download is in one thread); Note: there is a limit at which you will be throttled by GitHub, so if you are using that as your repository source you should put this number as low as possible. If it is from another service (such as an internal repository store) then setting this higher will speed up the process.

## PROJECTS_FILE
Specifies where the file containing the repositories which the program will clone. It will be read (line by line) until all the repositories have been downloaded

## LOGS_FOLDER 
Specifies the folder where the git logs will be copied to after the respository is downloaded.


# Additional Configuration
You can avoid deleting the downloaded project after the git log is extracted by **removing** the following line from `scripts/save_git_log.sh`

	$ rm -r $path_to_git

Be aware that with a large number of projects, this could be many terabytes of stored data and PHANTOM will start to error due to a lack of disk space to save the new projects.

# Running the tool
To run the tool you need the following command;

	$ cargo run --release

To get an executable to run later you need the following commands;

	$ cargo build --release
	$ cd ./target/release
	
The execuatable file should be called project_analyser in this folder (.exe extension for windows)
