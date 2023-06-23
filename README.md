# pacq
Simple "package manager" for rebuilding your system. Installs lists of programs/apps/libraries using provided package manager.

## running
pacq folder refers to the folder where config and batch files are
```
pacq run path-to-pacq-folder path-to-log-file
```

## executing
execute a single batch
```
pacq execute batch_file_name path-to-log-file
```

## creating template files
creates an empty batch or a config file with the given file_name (path)
```
pacq template batch/config file_name
```

## config file
```
{
    'batches': [
        'example',
        ...
    ]
}
```
batches field contains batch filenames without trailing .json  
batches are executed in order they are listed

## batch file
```
{
    "command": "pacman",
    "args": ["-S"],
    "forward_input": "Y",
    "items": ["rust"],
    "break_the_chain": true,
    "one_by_one": false
}
```
**command** - the name of the package manager the batch is ran against  
**args** -  flags given to the package manager  
\[optional\] **forward_input** - string to be put into the stdin once the package manager is ran (e.g. 'Y')  
**items** - programs/libraries to be installed via the package manager  
**break_the_chain** - if this batch fails do not continue with the other batches if there are any after this (also breaks the current batch if items are installed one by one)  
**one_by_one** - if yes all items will be installed separately instead of all at once