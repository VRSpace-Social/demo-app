/** @type {import('./$types').PageLoad} */
export async function load({ params, fetch }) {
	const userData = await fetch('http://localhost:3000/api/getUserInfo?userID=' + params.slug);
	if(userData) {
		const user: any = await userData.json();
		console.log(user);
		return {
			post: {
				title: user.displayName,
				content: user.bio,
				picture: user.currentAvatarImageUrl,
			}
		};
	
	}
	/*
	return {
		post: {
			title: `Title for ${params.slug} goes here`,
			content: `Content for ${params.slug} goes here`
		}
	};
	*/
}