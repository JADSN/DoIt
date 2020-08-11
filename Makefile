bintarget = ./target/x86_64-unknown-linux-musl/release/app
cmd = cargo build --release --target=x86_64-unknown-linux-musl -p app && strip $(bintarget) && $(bintarget)

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

# update_one:
# 	$(cmd) $(app_cli) update_one -i "5" -d "UPDATED" -o "true"

update_todo_done:
	$(cmd) $(app_cli) update_todo_done -i "3" -o "true"

update_todo_description:
	$(cmd) $(app_cli) update_todo_description -i "3" -d "LA BOLICHA"

delete_one:
	$(cmd) $(app_cli) delete_one -i "2"

help:
	$(cmd) --help

binsize:
	ls -lh $(bintarget)

dist_to_deploy:
	rm -rfv ./dist
	mkdir dist
	cp -vR ./static ./dist/.
	cp -v $(bintarget) ./dist/.
	cd ./dist && ./app $(app_cli) server