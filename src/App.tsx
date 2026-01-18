import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { CommercesPage } from "./pages/Commerces";
import { InspectionsPage } from "./pages/Inspections";
import { NewInspectionPage } from "./pages/NewInspection";
import { DashboardLayout } from "./layouts/DashboardLayout";
import "./App.css";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<DashboardLayout />}>
          <Route index element={<Navigate to="/dashboard" replace />} />
          <Route path="dashboard" element={<div className="p-10 font-arabic text-center text-gray-500">لوحة القيادة (قيد الإنجاز)</div>} />
          <Route path="commerces" element={<CommercesPage />} />
          <Route path="inspections" element={<InspectionsPage />} />
          <Route path="inspections/new" element={<NewInspectionPage />} />
        </Route>

        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
