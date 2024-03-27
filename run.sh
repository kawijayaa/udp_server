cargo build --release

sudo setcap cap_net_admin=eip target/release/udp_server

./target/release/udp_server &
pid=$!

sudo ip addr add 192.168.0.1/24 dev tun1
sudo ip link set up dev tun1

trap "kill $pid" INT TERM
wait $pid
