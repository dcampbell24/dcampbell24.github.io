.PHONY: website
website:
	mdbook build --dest-dir /var/www/html/
	touch /var/www/html/robots.txt
