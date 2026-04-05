import styles from './GalaxyCard.module.css';

interface GalaxyCardProps {
  name: string;
  balance: number;
}

export function GalaxyCard({ name, balance }: GalaxyCardProps) {
  const formattedBalance = new Intl.NumberFormat('pt-BR', {
    style: 'currency',
    currency: 'BRL',
  }).format(balance);

  return (
    <div className={styles.card}>
      <div className={styles.chip} />
      <div className={styles.info}>
        <span className={styles.label}>Saldo Disponível</span>
        <h2 className={styles.balance}>{formattedBalance}</h2>
      </div>
      <div className={styles.footer}>
        <span className={styles.holder}>{name}</span>
        <span className={styles.brand}>Giba Bank</span>
      </div>
    </div>
  );
}