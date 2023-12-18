The objective here was to find the first and last number of each string, concatenate them and sum all those up. For part 1, this was just with digits e.g. `5`, but part 2 also included words e.g. `five`

# Part 1

I used `find` and `rfind` to get the first and last numeric characters.
Then concatenated them, converted them to integers and summed them

# Part 2

I decided to try two methods here - a linear search and a string replace. I decided to overengineer this by using traits. I have a `NumberFinder` trait that defines a method to find the first number in a line, and specify if it should search from the start or end

## Linear Search

This was my first implementation. The idea is that it will search a growing substring until it finds either a word number or a digit

1. Loop through each character of the string
2. Look at the substring from the start up to and including this character
3. If this substring ends with a word number, return the value of that number
4. If the character at the end of the substring is a digit, return that
5. Otherwise, go on to the next character

This works similarly for searching from the end, except the substring is from the character position to the end, and the substring is checked for a word number at the start rather than at the end

## String Replace

This is the easier one to understand, but slightly less efficient. The idea is just to replace all word numbers with the corresponding digits and use the same code from part 1. The difficulty comes because some word numbers can overlap e.g. `twone`

1. Replace any overlapping word numbers with their separate numbers e.g. `twone` becomes `twoone`
2. Replace any word numbers with their corresponding digit e.g. `two` becomes `2`
3. Use the code from part 1 to search from the start/end of the string for the first digit
