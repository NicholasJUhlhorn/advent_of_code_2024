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
## Task Two
Now we need to calculate a similarity score: sum of elements in list 1 with their product of the number of times they are in list 2.
My plan is to create a count dictionary of list 2 and then use that to loop through list 1 and get the similarity score.
