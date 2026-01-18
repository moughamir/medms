import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { TotpSetupPage } from "./pages/TotpSetup";
import { CommercesPage } from "./pages/Commerces";
import { DashboardLayout } from "./layouts/DashboardLayout";
import "./App.css";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<TotpSetupPage />} />

        <Route element={<DashboardLayout />}>
          <Route path="/dashboard" element={<div className="p-10 font-arabic text-center text-gray-500">لوحة القيادة (قيد الإنجاز)</div>} />
          <Route path="/commerces" element={<CommercesPage />} />
          <Route path="/inspections" element={<div className="p-10 font-arabic text-center text-gray-500">المعاينات (قيد الإنجاز)</div>} />
        </Route>

        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
