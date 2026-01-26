/**
 * Example TypeScript module for spec extraction testing.
 */

export const MAX_RETRIES = 3;
export const DEFAULT_NAME = "Anonymous";

// Serializable interface for objects that can be serialized.
export interface Serializable {
    toJSON(): string;
    fromJSON(json: string): void;
}

// Greeter interface for greeting behavior.
export interface Greeter {
    greet(): string;
    farewell(): string;
}

// User represents a user in the system.
export interface User {
    id: number;
    name: string;
    email?: string;
}

// UserRole enum for user roles.
export enum UserRole {
    Admin = "admin",
    Editor = "editor",
    Viewer = "viewer"
}

// UserImpl is a class implementing User and Greeter.
export class UserImpl implements User, Greeter, Serializable {
    id: number;
    name: string;
    email?: string;

    constructor(id: number, name: string, email?: string) {
        this.id = id;
        this.name = name;
        this.email = email;
    }

    greet(): string {
        return `Hello, ${this.name}`;
    }

    farewell(): string {
        return `Goodbye, ${this.name}`;
    }

    toJSON(): string {
        return JSON.stringify({ id: this.id, name: this.name, email: this.email });
    }

    fromJSON(json: string): void {
        const data = JSON.parse(json);
        this.id = data.id;
        this.name = data.name;
        this.email = data.email;
    }
}

// Result type for operation results.
export type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };

// UserMap type alias.
export type UserMap = Map<number, User>;

// Create a new user with the given name.
export function createUser(name: string, email?: string): User {
    return {
        id: Math.floor(Math.random() * 10000),
        name,
        email
    };
}

// Arrow function for validation.
export const validateEmail = (email: string): boolean => {
    return email.includes("@") && email.includes(".");
};

// Generic function example.
export function findById<T extends { id: number }>(items: T[], id: number): T | undefined {
    return items.find(item => item.id === id);
}
