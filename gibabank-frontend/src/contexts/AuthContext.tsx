import { createContext, useState, useEffect } from 'react';
import type { ReactNode } from 'react';
import { api } from '../services/api';

interface User {
    id: string;
    full_name: string;
    email: string;
}

interface AuthContextData {
    user: User | null;
    signIn: (credentials: object) => Promise<void>;
    signOut: () => void;
    signed: boolean;
}

export const AuthContext = createContext<AuthContextData>({} as AuthContextData);

export function AuthProvider({ children }: { children: ReactNode }) {
    const [user, setUser] = useState<User | null>(null);

    useEffect(() => {
        const storagedUser = localStorage.getItem('@GibaBank:user');
        const storagedToken = localStorage.getItem('@GibaBank:token');

        if (storagedUser && storagedToken) {
            setUser(JSON.parse(storagedUser));
        }
    }, []);

    async function signIn(credentials: object) {
        const response = await api.post('/login', credentials);
        const { token, user } = response.data;

        setUser(user);
        localStorage.setItem('@GibaBank:token', token);
        localStorage.setItem('@GibaBank:user', JSON.stringify(user));
    }

    function signOut() {
        localStorage.clear();
        setUser(null);
    }

    return (
        <AuthContext.Provider value={{ signed: !!user, user, signIn, signOut }}>
            {children}
        </AuthContext.Provider>
    );
}