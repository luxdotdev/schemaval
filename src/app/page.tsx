"use client";

import { ClientOnly } from "@/components/client-only";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
} from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import { FormField, FormItem } from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { toast } from "@/components/ui/use-toast";
import { invoke } from "@/lib/invoke";
import { cn } from "@/lib/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { Form, useForm } from "react-hook-form";
import { z } from "zod";

const TXT = "text/plain";

const ACCEPTED_FILE_TYPES = [TXT]; // Add more file types as needed
const MAX_FILE_SIZE = 1000000; // 1MB in bytes

const formSchema = z.object({
  file: z
    .any()
    .refine((file) => file !== null && file !== undefined, "File is required.")
    .refine(
      (file) => file && file.size <= MAX_FILE_SIZE,
      "Max file size is 1MB."
    )
    .refine(
      (file) => file && ACCEPTED_FILE_TYPES.includes(file.type),
      ".txt files are accepted."
    ),
  source: z.enum(["ScrimTime"]),
  compatibility: z.enum(["Parsertime"]),
});

export default function Home() {
  const [dragActive, setDragActive] = useState(false);
  const router = useRouter();

  async function handleFile(file: File) {
    toast({
      title: "Creating map...",
      description: "We are processing your data. Please wait.",
      duration: 5000,
    });

    handleFormSubmit({
      file,
      source: form.getValues("source"),
      compatibility: form.getValues("compatibility"),
    });
  }

  function handleDrag(e: React.DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === "dragenter" || e.type === "dragover") {
      setDragActive(true);
    } else if (e.type === "dragleave") {
      setDragActive(false);
    }

    e.dataTransfer.dropEffect = "copy";
  }

  function handleDrop(e: React.DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);

    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      // at least one file has been selected so do something
      handleFile(e.dataTransfer.files[0]);
    }
  }

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
    e.preventDefault();
    if (e.target.files && e.target.files[0]) {
      // at least one file has been selected so do something
      handleFile(e.target.files[0]);
    }
  }

  async function handleFormSubmit(data: z.infer<typeof formSchema>) {
    const fileText: string = await data.file.text();

    await invoke("validate", {
      file: fileText,
      source: data.source,
      compatibility: data.compatibility,
    });
  }

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      file: null,
    },
  });

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
        <ClientOnly>
          <Form {...form}>
            <form className="w-full p-2 space-y-2" onDragEnter={handleDrag}>
              <FormField
                control={form.control}
                name="file"
                render={() => (
                  <FormItem>
                    <Card
                      className={cn(
                        "flex h-[60vh] flex-col items-center justify-center border-dashed",
                        dragActive && "border-green-500"
                      )}
                      onDragEnter={handleDrag}
                      onDragLeave={handleDrag}
                      onDragOver={handleDrag}
                      onDrop={handleDrop}
                    >
                      <CardHeader className="text-center text-xl">
                        <span className="inline-flex items-center justify-center space-x-2">
                          <span>Add a file...</span>
                        </span>
                      </CardHeader>
                      <CardDescription className="pb-4">
                        Drag and drop or select a file to upload.
                      </CardDescription>
                      <CardContent className="flex items-center justify-center">
                        <Label htmlFor="file" className="hidden">
                          Add a file
                        </Label>
                        <Input
                          id="file"
                          type="file"
                          onChange={handleChange}
                          className="w-64"
                          accept=".xlsx, .txt"
                        />
                        <div className="pl-2" />
                      </CardContent>
                    </Card>
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="source"
                defaultValue="ScrimTime"
                render={({ field }) => (
                  <FormItem>
                    <div className="flex items-center space-x-2">
                      <Label
                        htmlFor="source"
                        className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                      >
                        Source
                      </Label>
                      <Select
                        onValueChange={field.onChange}
                        defaultValue={"ScrimTime"}
                      >
                        <SelectTrigger className="w-[240px] pl-3 text-left font-normal">
                          <SelectValue placeholder="Select a source..." />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="ScrimTime">ScrimTime</SelectItem>
                        </SelectContent>
                      </Select>
                    </div>
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="compatibility"
                defaultValue="Parsertime"
                render={() => (
                  <FormItem>
                    <div className="flex items-center space-x-2">
                      <Checkbox id="compatibility" checked disabled />
                      <Label
                        htmlFor="compatibility"
                        className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                      >
                        Parsertime
                      </Label>
                    </div>
                  </FormItem>
                )}
              />
            </form>
          </Form>
        </ClientOnly>
      </div>
    </main>
  );
}
