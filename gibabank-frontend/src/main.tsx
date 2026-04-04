import { createRoot } from 'react-dom/client'
import { AuthProvider } from './contexts/AuthContext.tsx'
import './index.css'
import { Login } from './pages/Login/index.tsx'

createRoot(document.getElementById('root')!).render(
  <AuthProvider>
    <Login></Login>
  </AuthProvider>,
)
