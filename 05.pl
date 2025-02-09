use strict;
use warnings;

	my $user_role = $ARGV[0] || 'guest';  

	print "Welcome, your role is: $user_role\n";

	if ($user_role eq 'admin') {
   		 print " admin panel!\n";
	} else {
   		 print "user  panel!\n";
	}

