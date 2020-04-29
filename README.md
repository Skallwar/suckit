![](https://github.com/Skallwar/suckit/workflows/SuckIT%20Unit%20Tests%20CI/badge.svg)

# SuckIT

`SuckIT` allows you to recursively visit and download a website's content to
your disk.

# Features

* [x] Vacuum the entirety of a website recursively
* [x] Use multithreading
* [x] Write the website's content to your disk
* [x] Offline navigation

# Options

|Option|Behavior|
|---|---|
|`-h|--help`|Displays help information|
|`-v|--verbose`|Activate Verbose output|
|`-d|--depth`|Specify the level of depth to go to when visiting the website|
|`-j|--jobs`|Number of threads to use|
|`-o|--output`|Output directory where the downloaded files are written|
|`-t|--tries`|Number of times to retry when the downloading of a page fails|

# Example

A common use case could be the following:

`suckit http://books.toscrape.com -j 8 -o /path/to/downloaded/pages/
