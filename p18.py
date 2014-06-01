#!/usr/bin/python


# given row and and index at row+1, what elements can the
# index element see?
# example:
#  1 2 3
# 4 5 6 7
# here, 5 (index 1) can see 1 (index 0),2 (index 1)
#       6 (index 2) can see 2 (index 1), 3 (index 2)
# any node can see at most 2 elements on the row above it
def get_accessible(row, index):
	row_len = len(row)
	ret_val = list()
	if index-1 >= 0:
		ret_val.append(row[index-1])
	if index < row_len:
		ret_val.append(row[index])
	return ret_val
	

def explore(current_row, previous_row):
	row_len = len(current_row)
	for i in enumerate(current_row):
		if 0 == i:
			# only way to get to elem 0 is through the 1st node
			current_row[0] += previous_row[0] 
		elif row_len-1 == i:
			current_row[row_len-1] += previous_row[row_len-2]
		else: # all the middle elements
			accessible = get_accessible(previous_row, i[0] )
			current_row[i[0]] += max(accessible)

def main():
	file = open("i67")
	lines = file.readlines()
	
	# get a 2D jagged array of the input first
	array = list()
	for line in lines:
		array.append([int(num) for num in line.split()])

	for i in range(0, len(array)-1):
		explore(array[i+1], array[i])

	array[len(array)-1].sort()
	print(array[len(array)-1])

if "__main__" == __name__:
	main()
