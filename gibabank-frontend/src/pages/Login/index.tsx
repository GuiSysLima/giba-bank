import { useState, useContext, type FormEvent } from 'react';
import { AuthContext } from '../../contexts/AuthContext';
import styles from './Login.module.css';

import galaxyVideo from '../../assets/galaxy_bg.mp4';

export function Login() {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');

    const { signIn } = useContext(AuthContext);

    async function handleSubmit(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        setError('');

        try {
            await signIn({ email, password });
            alert("Login realizado com sucesso!");
        } catch (err) {
            setError('E-mail ou senha inválidos. Tente novamente.');
            console.error(err);
        }
    }

    return (
        <main className={styles.mainContainer}>
            <video
                autoPlay
                loop
                muted
                playsInline
                className={styles.videoBackground}
            >
                <source src={galaxyVideo} type="video/mp4" />
                Seu navegador não suporta vídeos HTML5.
            </video>
            <div className={styles.contentWrapper}>
                <div className={styles.card}>
                    <h1 className={styles.title}>Giba Bank</h1>
                    <p className={styles.subtitle}>Acesse sua conta espacial</p>

                    <form className={styles.form} onSubmit={handleSubmit}>
                        <div className={styles.inputGroup}>
                            <label>E-mail</label>
                            <input
                                type="email"
                                placeholder="exemplo@email.com"
                                value={email}
                                onChange={(e) => setEmail(e.target.value)}
                                required
                            />
                        </div>

                        <div className={styles.inputGroup}>
                            <label>Senha</label>
                            <input
                                type="password"
                                placeholder="••••••••"
                                value={password}
                                onChange={(e) => setPassword(e.target.value)}
                                required
                            />
                        </div>

                        <button type="submit" className={styles.button}>
                            Entrar na Órbita
                        </button>
                    </form>

                    {error && <p className={styles.error}>{error}</p>}
                </div>
            </div>
        </main>
    );
}