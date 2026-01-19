import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { useAuthStore } from './stores/authStore';
import { LoginPage } from './pages/Login';
import { CommercesPage } from "./pages/Commerces";
import { InspectionsPage } from "./pages/Inspections";
import { NewInspectionPage } from "./pages/NewInspection";
import { DashboardLayout } from "./layouts/DashboardLayout";
import { SettingsPage } from "./pages/Settings";
import "./App.css";

const AuthGuard = ({ children }: { children: React.ReactNode }) => {
  const { isAuthenticated } = useAuthStore();
  return isAuthenticated ? <>{children}</> : <Navigate to="/login" replace />;
};

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/login" element={<LoginPage />} />

        <Route path="/" element={
          <AuthGuard>
            <DashboardLayout />
          </AuthGuard>
        }>
          <Route index element={<Navigate to="/dashboard" replace />} />
          <Route path="dashboard" element={<div className="p-10 font-arabic text-center text-gray-500">لوحة القيادة (قيد الإنجاز)</div>} />
          <Route path="commerces" element={<CommercesPage />} />
          <Route path="inspections" element={<InspectionsPage />} />
          <Route path="inspections/new" element={<NewInspectionPage />} />
          <Route path="settings" element={<SettingsPage />} />
        </Route>

        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
