# Reading and writing files

Most of the data required to do analysis is either stored in a file or a
database. For this reason it makes sense to cover the different methods
available in the Rust Arrow implementation to extract and save information
to a data file.

This section of the guide has three chapters discussing the available methods to
read data stored in files. The types of files that we are going to cover are
csv, json and parquet files. Hopefully by the end of this section you will
be familiar enough with the available interfaces to interact with files.