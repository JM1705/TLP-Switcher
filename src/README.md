Some stuff requires root so do these things for tlp-switcher to work:

run:
sudo chmod a+w /etc/tlp.d

and add:
my_username ALL = NOPASSWD:/usr/bin/tlp
to /etc/sudoers
