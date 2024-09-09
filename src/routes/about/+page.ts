// src/routes/user/[id]/+page.server.ts
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ locals, params }) => {
    const user = locals.user; // Access the user data
    if (!user) {
        throw new Error('User not found'); // Handle case where user data is not present
    }
    return {
        user, // Return user data to the page
    };
};