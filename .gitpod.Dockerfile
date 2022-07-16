FROM gitpod/workspace-full:latest
RUN docker run -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres