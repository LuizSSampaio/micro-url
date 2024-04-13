
# Micro Url

> **WARNING:** DON'T USE IN PRODUCTION! THIS PROJECT IS ONLY FOR STUDY

Micro URL is a streamlined and minimalist backend solution for link shortening. Its simplicity and lightweight design make it an ideal choice for efficiently managing and shortening URLs without unnecessary overhead.

## Lessons Learned

Throughout this project, I gained invaluable insights into leveraging Rust for database operations, delving into asynchronous programming paradigms, and comprehending the inner workings of a link shortening service. These experiences not only broadened my understanding of Rust's capabilities but also honed my proficiency in handling data persistence and asynchronous workflows.

## Run Locally

Clone the project

```bash
  git clone https://github.com/LuizSSampaio/micro-url.git
```

Go to the project directory

```bash
  cd micro-url
```

Start the server

```bash
  cargo run
```

## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`DATABASE_URL`

### Exemple

```
DATABASE_URL=file:test.db
```

## API Reference

#### Redirect

```http
  GET /r/${url_id}
```

| Parameter | Type     | Description                                       |
| :-------- | :------- | :------------------------------------------------ |
| `url_id`  | `string` | **Required**. The url_id to identify the long url |


#### Create new short url

```http
  POST /api/create
```

| Parameter  | Type     | Description                         |
| :--------- | :------- | :---------------------------------- |
| `long_url` | `string` | **Required**. The url to be shorted |


#### Change the long url from the short url

```http
  PUT /api/edit
```

| Parameter     | Type     | Description                                          |
| :------------ | :------- | :--------------------------------------------------- |
| `url_id`      | `string` | **Required**. The url_id to identify the url to edit |
| `long_url`    | `string` | **Required**. The new long url                       |


#### Delete a short url

```http
  DELETE /api/delete
```

| Parameter | Type     | Description                                            |
| :-------- | :------- | :----------------------------------------------------- |
| `url_id`  | `string` | **Required**. The url_id to identify url to be deleted |


## Roadmap

- Change project architecture

- Create tests

- Add authentication system

- Change sqlite to postgres


