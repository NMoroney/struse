# [2604] n8
#

#
# * rustup default stable
#


all :
	rustc struse.rs
	echo "<html>" > t1.txt
	./struse >> t1.txt
	echo "</html>" >> t1.txt
	awk '{print("<br>")}' < t1.txt > t2.txt
	paste t1.txt t2.txt > index.html
	rm t*.txt

