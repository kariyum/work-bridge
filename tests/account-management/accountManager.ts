import { UserAccount } from '../models/user-account';
import { cyrb53 } from '../lib/utils.js';

export async function createTestAccount(accountType = 'recruiter') {
    const userAccount : UserAccount = {
        email: `test_${Date.now()}@gmail.com`,
        password: 'Test@1234',
        first_name: `test_firstname`,
        last_name: `test_lastname`
    };
    const formParams = new URLSearchParams({
        role: accountType,
        email: userAccount.email,
        password: cyrb53(userAccount.password).toString(),
        first_name: userAccount.first_name,
        last_name: userAccount.last_name
    });
    const response = await fetch('http://localhost:8080/auth/accounts', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: formParams.toString(),
    });

    if (response.ok) {
        return userAccount;
    } else {
        throw new Error(`Failed to create ${accountType} account: ${response.status}`);
    }
}

export async function deleteTestAccount(accountId: string) {
    const response = await fetch(`http://localhost:8080/auth/accounts/${accountId}`, {
        method: 'DELETE'
    });
    if (!response.ok) {
        throw new Error(`Failed to delete account ${accountId}: ${response.status}`);
    }
    else {
        console.log(`Account ${accountId} deleted successfully`);
    }
}
