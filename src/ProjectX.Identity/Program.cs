using System.Text.Json.Serialization;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services
    .AddControllers()
    .AddDapr();

// Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();
app.UseAuthorization();
app.MapControllers();
app.MapSubscribeHandler();
app.UseCloudEvents();

app.MapPost("/orders", (Order order) =>
{
    Console.WriteLine("Subscriber received : " + order);
    return Results.Ok(order);
}).WithTopic("pubsub", "orders");

app.MapGet("/", () => "Hello from Auth");

app.Run();

public record Order([property: JsonPropertyName("orderId")] int OrderId);