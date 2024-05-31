import { BottomLinks } from "@/components/bottom-links";
import { InputForm } from "@/components/form";

export default function Home() {
  return (
    <main className="h-screen p-2">
      <div className="h-full p-2">
        <div className="flex items-center justify-center py-2">
          <h1 className="text-2xl font-bold tracking-tight">
            Log Schema Validator
          </h1>
        </div>
        <p className="flex items-center justify-center pb-2">
          This app checks an Overwatch log file to see if it is compatible with
          the Parsertime web app.
        </p>
        <InputForm />
        <BottomLinks />
      </div>
    </main>
  );
}
