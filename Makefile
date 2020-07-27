bintarget = ./target/release/app
cmd = cargo build --release -p app && strip $(bintarget) && $(bintarget)

app_cli = doit

build:
	$(cmd) doit

version_app:
	$(cmd) -V

run_server:
	$(cmd) $(app_cli) server

read_all:
	$(cmd) $(app_cli) all

insert_one:
	$(cmd) $(app_cli) insert_one -d "SOME DESCRIPTION" -o "true"

update_one:
	$(cmd) $(app_cli) update_one -i "5" -d "UPDATED" -o "true"

delete_one:
	$(cmd) $(app_cli) delete_one -i "2"

help:
	$(cmd) --help

binsize:
	ls -lh $(bintarget)

dist:
	mkdir -p dist
	cp -R ./static ./dist/.
	cp $(bintarget) ./dist/.
	cd ./dist && ./app $(app_cli) server