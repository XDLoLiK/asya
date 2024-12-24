pub const PROMPT_STRING: &'static str = r#"
I'm going to provide you with a user command line input,
and I expect you to reply in a very strict format.

The format:
    1. Your output must contain exactly "n" words, where "n" varies with each command;
    2. The response cannot contain less than one word. The first word is always a command name;
    3. All the other words are command arguments, which you have to deduce from the user input;
    4. Each argument must be a single word, there are only two types of the arguments;
    5. Do not pay attention to the names of the arguments, only to their types and amount;
    6. The arguments may appear in any order, but you should reorder them to get a correct command.

Argument types:
    1. string
        Must be a single word;

    2. bool
        Only true or false.

Avaliable commands:
    1. remove command
        n: 3
        arguments: 1) remove path (string); 2) force remove (bool);
        description: remove a file or directory,
            force flag removes a directory and its contents recursively 

    2. create command
        n: 2
        arguments: 1) create path (string);
        description: create a new file or directory

    3. copy command
        n: 3
        arguments: 1) source file: (string); 2) destination file (string);
        description: copies the contents of one file to another

    4. rename command
        n: 3
        arguments: 1) source path (string); 2) destination path (string);
        description: rename a file or directory to a new name

    5. move command
        n: 3
        arguments: 1) source path (string); 2) destination path (string);
        description: move a file or directory to a new place

    6. find command
        n: 3
        arguments: 1) folder to search in (string); 2) name to search for (string).
        description: find a file or directory

Examples:
    1. User input: remove file named test.txt;
        Expected answer: remove test.txt false

    2. User input: create folder debug and put a file named test txt there;
        Expected answer: create ./debug/test.txt

User input:
"#;
