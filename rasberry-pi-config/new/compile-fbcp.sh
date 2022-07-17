sudo apt-get install cmake -y
sudo apt-get install git -y
cd ~
git clone https://github.com/mpatrois/Waveshare_fbcp.git
cd ~/Waveshare_fbcp

sudo cmake -Bbuild -DSPI_BUS_CLOCK_DIVISOR=8 -DWAVESHARE_2INCH4_LCD=ON -DBACKLIGHT_CONTROL=OFF -DSTATISTICS=2
sudo cmake --build build --target
if [ -x "/usr/local/bin/fbcp" ]; then
sudo rm -rf /usr/local/bin/fbcp
fi
sudo cp ~/Waveshare_fbcp/fbcp /usr/local/bin/fbcp