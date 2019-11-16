debian-keyboard-2019-11.pdf: debian-keyboard-2019-11.dvi
	dvipdfmx $<

debian-keyboard-2019-11.dvi: debian-keyboard-2019-11.tex
	latexmk

.PHONY: clean
clean:
	$(RM) debian-keyboard-2019-11.pdf
	$(RM) debian-keyboard-2019-11.dvi
