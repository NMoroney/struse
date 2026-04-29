# [2604] n8
#

all :
	echo "<html>" > t1.txt
	python3 struse.py >> t1.txt
	echo "</html>" >> t1.txt
	awk '{print("<br>")}' < t1.txt > t2.txt
	paste t1.txt t2.txt > index.html
	rm t*.txt

