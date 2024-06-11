import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {	
        return {			
            title: 'Hello world!',
        	content: 'Welcome to our blog. Lorem ipsum dolor sit amet...',
        };	
    }