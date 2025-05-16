list:
	@just -l
update:
	cargo build --release && sudo docker build -t tiger-bot . && sudo docker save -o tiger_bot_image tiger-bot && sudo scp -i /home/cbarnes/.ssh/id_ed25519 ./tiger_bot_image cameron@orbitaltiger:~/tiger_bot/tiger_bot_image
