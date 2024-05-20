import { exitStatus, asMain } from 'std';

export function greet(name: string): string {
	return "Hello, " + name "!";
}

export function main(): void {
	greet("Angela");
}

if (asMain("main")){
	main();
}
else {
	exitStatus(0);
}
