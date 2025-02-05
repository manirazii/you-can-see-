#include<stdio.h>
#include<string.h>

int main(){
	char passwd[10];
	char test[10];
	
	strcpy(passwd,"welcome");
	
	gets(test);
	

	if(!strncmp(passwd,test,10)){
		printf("welcome to my app");}

	 else{
	 	printf("who are you?");
	 }


	return 0;
}
