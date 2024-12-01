# advent_of_code_2024
Reupping my rust skills again over December.\\

Following is my thought process for each day:
# Day One
## Task One
First idea is to take in the input to make two lists to operate upon.
* Populate lists
* Sort lists (min to max)
* Take differences of lists and add to sum
The sum will be the answer. Because I am sorting the lists we have a O(nlog(n)) runtime, I do not think there is an improvement.
