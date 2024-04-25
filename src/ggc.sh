if [[ "$1" == "--uninstall" ]]; then
	sudo rm /bin/ggc-bin
	sudo rm /bin/ggc
elif [[ "$1" == "--help" ]]; then
	echo "For uninstalling ggc type \"ggc --uninstall\""
else
	ggc-bin; 
fi
